from rest_framework.filters import BaseFilterBackend
from rest_framework.settings import api_settings


class RandomOrderFilter(BaseFilterBackend):
    """
    Filter backend that allows random ordering of queryset based on the 'random' parameter in the request query parameters.

    ! This filter backend should be at the end of the 'filter_backends' list to work properly.

    Usage:
    - Add 'RandomOrderFilter' to the 'filter_backends' list in your view.

    Example:
    class MyView(APIView):
        filter_backends = [RandomOrderFilter]
        # optional
        random_query_param = "random"
        ...
    """

    ordering_param = api_settings.ORDERING_PARAM
    random_query_param = "random"

    def filter_queryset(self, request, queryset, view):
        ordering = request.query_params.get(self.ordering_param, "")
        params = [param.strip() for param in ordering.split(",")]

        if self.random_query_param in params:
            return queryset.order_by("?")

        return queryset
