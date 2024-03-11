from django.db import models


class BaseModel(models.Model):
    """
    Base model class that provides common fields for all models.
    """

    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        abstract = True
