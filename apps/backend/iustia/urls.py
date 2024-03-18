from django.contrib import admin
from django.urls import include, path
from rest_framework.routers import DefaultRouter

from iustia.jobs.views import CompanyViewSet, JobViewSet

router = DefaultRouter()
router.register(r"jobs", JobViewSet)
router.register(r"companies", CompanyViewSet)

urlpatterns = [
    path("admin/", admin.site.urls),
    path("api/v1/", include(router.urls)),
]
