from django.contrib import admin

from .models import Company, CompanyOffices, Job

admin.site.register(Company)
admin.site.register(CompanyOffices)
admin.site.register(Job)
