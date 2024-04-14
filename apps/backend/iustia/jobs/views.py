from typing import Optional

from django_filters import rest_framework as filters
from rest_framework import viewsets
from rest_framework.decorators import action
from rest_framework.request import Request
from rest_framework.response import Response

from iustia.core.filters import RandomOrderFilter

from .models import Company, Job, JobApplication
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

    @action(detail=True, methods=["POST"])
    def apply(self, request: Request, pk: Optional[int] = None) -> Response:
        print(request.user)
        if request.user.is_anonymous:
            return Response(
                {"detail": "Authentication credentials were not provided."}, status=403
            )

        job = self.get_object()  # type: Job

        if JobApplication.objects.filter(job=job, user=request.user).exists():
            return Response(
                {"detail": "You have already applied for this job."}, status=400
            )

        JobApplication.objects.create(job=job, user=request.user)

        return Response({"detail": "Application submitted."}, status=200)


class CompanyViewSet(viewsets.ModelViewSet):
    queryset = Company.objects.all()
    serializer_class = CompanySerializer
