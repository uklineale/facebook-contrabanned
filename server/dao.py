from pynamodb.models import Model
from pynamodb.attributes import UnicodeAttribute


class First(Model):
    class Meta:
        table_name = 'First'
        region = 'us-west-1'
    test_field = UnicodeAttribute(hash_key=True)

# if not First.exists():
First.create_table(wait=True, read_capacity_units=10, write_capacity_units=10)

first_item = First('test')
first_item.save()

#Why doesn't this do anything?