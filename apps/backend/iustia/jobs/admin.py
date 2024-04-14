from django.contrib import admin

from .models import Company, CompanyOffices, Job, JobApplication, JobImage


class JobImageInline(admin.TabularInline):
    model = JobImage
    extra = 0


@admin.register(Job)
class JobAdmin(admin.ModelAdmin):
    inlines = [JobImageInline]
    filter_horizontal = ("tags",)


@admin.register(JobApplication)
class JobApplicationAdmin(admin.ModelAdmin):
    list_display = ("job", "user", "created_at")
    search_fields = (
        "job__title",
        "user__email",
        "user__first_name",
        "user__last_name",
        "user__username",
    )


admin.site.register(Company)
admin.site.register(CompanyOffices)
