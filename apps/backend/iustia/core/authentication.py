import hashlib
import hmac
from operator import itemgetter
from typing import Optional, Tuple
from urllib.parse import parse_qsl

import pydantic
import pydantic.alias_generators
from django.conf import settings
from django.contrib.auth import get_user_model
from django.contrib.auth.models import AbstractBaseUser
from drf_spectacular.extensions import OpenApiAuthenticationExtension
from rest_framework import HTTP_HEADER_ENCODING
from rest_framework.authentication import BaseAuthentication
from rest_framework.request import Request


class TMAAuthData(pydantic.BaseModel):
    id: int
    first_name: str
    last_name: str
    language_code: str
    username: str


class TelegramMiniAppAuth(BaseAuthentication):
    keyword = "tma"
    user = get_user_model()

    def authenticate(self, request: Request) -> Optional[Tuple[AbstractBaseUser, None]]:
        header = self._get_header(request)
        if header is None:
            return None

        token = self._get_token(header)
        if token is None:
            return None

        auth_data = self._validate_token(token)

        if auth_data is None:
            return None

        return self._register_or_login(auth_data)

    def _get_header(self, request: Request) -> Optional[bytes]:
        """
        Extracts the header containing the JSON web token from the given
        request.
        """
        header = request.META.get("HTTP_AUTHORIZATION")

        if isinstance(header, str):
            header = header.encode(HTTP_HEADER_ENCODING)
            return header

        return None

    def _get_token(self, header: bytes) -> Optional[bytes]:
        """
        Extracts the JSON web token from the given header.
        """
        parts = header.split()

        if parts[0].lower() != self.keyword.lower().encode():
            return None

        if len(parts) != 2:
            raise ValueError("Invalid token header. No credentials provided.")

        return parts[1]

    def _register_or_login(
        self, auth_data: TMAAuthData
    ) -> Tuple[AbstractBaseUser, None]:
        """
        Registers or logs in the user with the given token.
        """
        telegram_user_id = self._get_telegram_user_id(auth_data.id)
        telegram_username = auth_data.username

        try:
            user = self.user.objects.get(id=telegram_user_id)
        except self.user.DoesNotExist:
            user = self.user(id=telegram_user_id, username=telegram_username)
            user.save()

        return user, None

    def _get_telegram_user_id(self, user_id: int) -> int:
        """
        Returns the Telegram user ID for the given user ID.
        """
        return user_id + 1_000_000_000

    def _validate_token(self, token: bytes) -> Optional[TMAAuthData]:
        """
        Validates the data received from the Telegram web app, using the
        method documented here:
        https://core.telegram.org/bots/webapps#validating-data-received-via-the-web-app
        """
        try:
            parsed_data = dict(parse_qsl(token.decode()))
        except ValueError:
            # Init data is not a valid query string
            return None

        auth_hash = parsed_data.pop("hash")
        if not auth_hash:
            return None

        data_check_string = "\n".join(
            f"{k}={v}" for k, v in sorted(parsed_data.items(), key=itemgetter(0))
        )

        bot_token = settings.TELEGRAM_BOT_TOKEN.encode()

        secret_key = hmac.new(
            key=b"WebAppData", msg=bot_token, digestmod=hashlib.sha256
        )
        calculated_hash = hmac.new(
            key=secret_key.digest(),
            msg=data_check_string.encode(),
            digestmod=hashlib.sha256,
        ).hexdigest()

        if not hmac.compare_digest(calculated_hash, auth_hash):
            # TODO: add logging
            return None

        return pydantic.TypeAdapter(TMAAuthData).validate_json(parsed_data["user"])


class TelegramMiniAppAuthScheme(OpenApiAuthenticationExtension):
    target_class = TelegramMiniAppAuth
    name = "TelegramMiniAppAuth"

    def get_security_definition(self, auto_schema):
        return {
            "type": "apiKey",
            "in": "header",
            "name": "tma",
        }
