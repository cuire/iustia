from django_filters import rest_framework as filters
from rest_framework import viewsets

from iustia.core.filters import RandomOrderFilter

from .models import Company, Job
from .serializers import CompanySerializer, JobSerializer


class JobFilterSet(filters.FilterSet):
    class Meta:
        model = Job
        fields = ["company", "job_type"]


class JobViewSet(viewsets.ModelViewSet):
    queryset = Job.objects.all()
    serializer_class = JobSerializer
    filter_class = JobFilterSet
    filter_backends = (filters.DjangoFilterBackend, RandomOrderFilter)

    page_size_query_param = "page_size"
    max_page_size = 100


class CompanyViewSet(viewsets.ModelViewSet):
    queryset = Company.objects.all()
    serializer_class = CompanySerializer
