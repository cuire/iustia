# Generated by Django 5.0.2 on 2024-03-20 15:14

from django.db import migrations


class Migration(migrations.Migration):
    dependencies = [
        ("jobs", "0002_jobimage"),
    ]

    operations = [
        migrations.AlterModelOptions(
            name="jobimage",
            options={"verbose_name": "Job Image", "verbose_name_plural": "Job Images"},
        ),
    ]
