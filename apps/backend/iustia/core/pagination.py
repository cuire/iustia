from rest_framework.pagination import PageNumberPagination


class StandardResultsSetPagination(PageNumberPagination):
    """
    Standard pagination class that allows setting the page size via the 'page_size' query parameter.
    https://www.example.com/api/v1/jobs/?page=1&page_size=10
    """

    page_size_query_param = "page_size"
    max_page_size = 1000
