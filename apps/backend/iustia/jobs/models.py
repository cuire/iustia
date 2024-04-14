from django.contrib.auth import get_user_model
from django.db import models
from django.utils.translation import gettext_lazy as _

from iustia.core.models import BaseModel
from iustia.jobs.validators import TrustedImageHostValidator

User = get_user_model()


class Company(BaseModel):
    name = models.CharField(
        _("Name"),
        max_length=255,
        blank=False,
        null=False,
    )
    email = models.EmailField(
        _("Email"),
        max_length=255,
        blank=False,
        null=False,
    )
    phone = models.CharField(
        _("Phone"),
        max_length=255,
        blank=False,
        null=False,
    )
    website = models.URLField(
        _("Website"),
        max_length=255,
        blank=False,
        null=False,
    )
    is_active = models.BooleanField(
        _("Is Active"),
        default=True,
    )

    class Meta:
        verbose_name = _("Company")
        verbose_name_plural = _("Companies")

    def __str__(self):
        return self.name


class CompanyOffices(BaseModel):
    company = models.ForeignKey(
        "jobs.Company",
        verbose_name=_("Company"),
        on_delete=models.CASCADE,
        related_name="offices",
    )
    address = models.TextField(
        _("Address"),
        blank=False,
        null=False,
    )

    class Meta:
        verbose_name = _("Company Office")
        verbose_name_plural = _("Company Offices")

    def __str__(self):
        return f"{self.company} - {self.address}"


JOB_TYPE = (
    ("full_time", _("Full Time")),
    ("part_time", _("Part Time")),
    ("contract", _("Contract")),
    ("internship", _("Internship")),
    ("temporary", _("Temporary")),
    ("volunteer", _("Volunteer")),
)


class Job(BaseModel):
    title = models.CharField(
        _("Title"),
        max_length=255,
        blank=False,
        null=False,
    )
    short_description = models.CharField(
        _("Short Description"),
        max_length=255,
        blank=False,
        null=False,
    )
    description = models.TextField(
        _("Description"),
        blank=False,
        null=False,
    )
    job_type = models.CharField(
        _("Job Type"),
        max_length=255,
        choices=JOB_TYPE,
        default="full_time",
    )
    experience = models.IntegerField(
        _("Experience"),
        help_text=_("Years of experience required"),
        default=0,
    )
    company = models.ForeignKey(
        "jobs.Company",
        verbose_name=_("Company"),
        on_delete=models.CASCADE,
        related_name="jobs",
    )
    location = models.ForeignKey(
        "jobs.CompanyOffices",
        verbose_name=_("Location"),
        on_delete=models.SET_NULL,
        related_name="jobs",
        blank=True,
        null=True,
    )
    is_remote = models.BooleanField(
        _("Is Remote"),
        default=False,
    )
    salary_min = models.DecimalField(
        _("Minimum Salary"),
        max_digits=10,
        decimal_places=2,
        default=0,
    )
    salary_max = models.DecimalField(
        _("Maximum Salary"),
        max_digits=10,
        decimal_places=2,
        default=0,
    )
    manager = models.ForeignKey(
        User,
        verbose_name=_("Manager"),
        on_delete=models.SET_NULL,
        related_name="jobs",
        blank=True,
        null=True,
    )
    is_active = models.BooleanField(
        _("Is Active"),
        default=True,
    )

    tags = models.ManyToManyField(
        "tags.Tag",
        verbose_name=_("Tags"),
        related_name="jobs",
        blank=True,
    )

    class Meta:
        verbose_name = _("Job")
        verbose_name_plural = _("Jobs")

    def __str__(self):
        return f"{self.company} - {self.title}"


class JobImage(BaseModel):
    job = models.ForeignKey(
        "jobs.Job",
        on_delete=models.CASCADE,
        related_name="images",
    )
    image_url = models.URLField(
        _("Image URL"),
        blank=False,
        null=False,
        validators=[TrustedImageHostValidator()],
    )

    class Meta:
        verbose_name = _("Job Image")
        verbose_name_plural = _("Job Images")

    def __str__(self):
        return f"{self.job} - {self.image_url}"


class JobApplication(BaseModel):
    job = models.ForeignKey(
        "jobs.Job",
        verbose_name=_("Job"),
        on_delete=models.CASCADE,
        related_name="applications",
    )

    user = models.ForeignKey(
        User,
        verbose_name=_("User"),
        on_delete=models.CASCADE,
        related_name="applications",
    )

    class Meta:
        verbose_name = _("Job Application")
        verbose_name_plural = _("Job Applications")
        unique_together = ("job", "user")

    def __str__(self):
        return f"{self.job} - {self.user}"
