from django.test import TestCase
from django.utils import timezone

from ..models import BaseModel


class BaseModelTestCase(TestCase):
    def test_created_at_auto_now_add(self):
        """
        Test that the `created_at` field is automatically set to the current time when a new instance is created.
        """
        instance = BaseModel.objects.create()
        self.assertIsNotNone(instance.created_at)
        self.assertLessEqual(instance.created_at, timezone.now())

    def test_updated_at_auto_now(self):
        """
        Test that the `updated_at` field is automatically updated to the current time when an instance is updated.
        """
        instance = BaseModel.objects.create()
        initial_updated_at = instance.updated_at
        instance.save()
        self.assertGreater(instance.updated_at, initial_updated_at)
        self.assertLessEqual(instance.updated_at, timezone.now())
