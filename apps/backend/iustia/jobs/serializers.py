from rest_framework import serializers

from .models import Company, Job, JobImage


class CompanySerializer(serializers.ModelSerializer):
    class Meta:
        model = Company
        fields = [
            "id",
            "name",
            "email",
            "phone",
            "website",
            "is_active",
        ]


class JobImageSerializer(serializers.ModelSerializer):
    class Meta:
        model = JobImage
        fields = [
            "image_url",
        ]


class JobSerializer(serializers.ModelSerializer):
    images = JobImageSerializer(many=True, read_only=True)

    class Meta:
        model = Job
        fields = [
            "id",
            "title",
            "description",
            "company",
            "location",
            "job_type",
            "is_active",
            "images",
        ]
