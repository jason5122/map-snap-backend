import requests
import json

api = 'http://localhost:3000'

r = requests.get(api)
r.status_code
print(r.status_code)
print(r.text)

payload = {'username': 'Jason1923'}
r = requests.post(api + '/users', json=payload)

response = r.json()
print(response['username'])

if response['username'] == 'Jason1923':
    print('success')
else:
    print('failure')
