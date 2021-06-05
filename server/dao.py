import pynamodb.exceptions
from pynamodb.models import Model
from pynamodb.attributes import UnicodeAttribute
from pynamodb.exceptions import DoesNotExist
import uuid

class RedirectSet(Model):
    class Meta:
        table_name = 'RedirectSets'
        region = 'us-west-2'
    id = UnicodeAttribute(hash_key=True)
    real_url = UnicodeAttribute()
    fake_url = UnicodeAttribute()

if not RedirectSet.exists():
    out = RedirectSet.create_table(wait=True, read_capacity_units=10, write_capacity_units=10)
    print(str(out))

def insert(real_url, fake_url):
    id = str(uuid.uuid4())
    new_item = RedirectSet(id, real_url=real_url, fake_url=fake_url)
    new_item.save()
    return id

def get(id):
    try:
        results = RedirectSet.get(id)
    except DoesNotExist:
        return None
    for item in results:
        return item



