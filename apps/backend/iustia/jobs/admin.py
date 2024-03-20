from django.contrib import admin

from .models import Company, CompanyOffices, Job, JobImage


class JobImageInline(admin.TabularInline):
    model = JobImage
    extra = 0


@admin.register(Job)
class JobAdmin(admin.ModelAdmin):
    inlines = [JobImageInline]


admin.site.register(Company)
admin.site.register(CompanyOffices)
