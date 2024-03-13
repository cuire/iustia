from django.test import TestCase


class BaseModelTestCase(TestCase):
    def test_created_at_auto_now_add(self):
        """
        Test that the `created_at` field is automatically set to the current time when a new instance is created.
        """
        # TODO: Implement this test
        ...

    def test_updated_at_auto_now(self):
        """
        Test that the `updated_at` field is automatically updated to the current time when an instance is updated.
        """
        # TODO: Implement this test
        ...
