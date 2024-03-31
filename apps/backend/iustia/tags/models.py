from django.db import models
from django.utils.text import slugify
from django.utils.translation import gettext_lazy as _

from iustia.core.models import BaseModel


class Tag(BaseModel):
    """
    Model used to store tags for the vacancies.
    Also used as an interest tag for the users.

    Example:
    - Python, Django, Flask, FastAPI, etc.
    """

    tag = models.CharField(
        _("Tag"),
        max_length=255,
        blank=False,
        null=False,
    )
    slug = models.SlugField(
        _("Slug"),
        max_length=255,
        blank=True,
        null=True,
        unique=True,
    )

    class Meta:
        verbose_name = _("Tag")
        verbose_name_plural = _("Tags")

    def save(self, *args, **kwargs) -> None:
        if not self.slug:
            self.slug = slugify(self.tag)

        super().save(*args, **kwargs)

    def __str__(self) -> str:
        return self.tag
