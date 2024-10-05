from django.shortcuts import render
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
import json

# Create your views here.
@csrf_exempt
def json_handler(request):
    if request.method == 'POST':
        try:
            data = json.loads(request.body)
            response = {
                'status': 'success',
                'data': data
            }
        except json.JSONDecodeError:
            response = {
                'status': 'error',
                'message': 'Invalid JSON data'
            }
    else:
        response = {
            'status': 'error',
            'message': 'Only POST requests are allowed'
        }
    return JsonResponse(response)
