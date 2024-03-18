from rest_framework import serializers

from .models import Company, Job


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


class JobSerializer(serializers.ModelSerializer):
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
        ]
