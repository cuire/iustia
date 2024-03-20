from urllib.parse import urlparse

from django.conf import settings
from django.core.validators import BaseValidator
from django.utils.translation import gettext_lazy as _


class TrustedImageHostValidator(BaseValidator):
    message = _(
        "The image URL: %(show_value)s is not from a trusted host. "
        "Please use a trusted image host from %(limit_value)s."
    )

    def __init__(self, message: str | None = None) -> None:
        limit_value = settings.TRUSTED_IMAGE_HOSTS
        super().__init__(limit_value=limit_value, message=message)

    def compare(self, value: str, trusted_hosts: list[str]) -> bool:
        if value and value in trusted_hosts:
            return False

        return True

    def clean(self, value: str) -> str | None:
        parsed_url = urlparse(value)
        return parsed_url.hostname
