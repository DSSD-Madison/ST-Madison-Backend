# Repository Layer

The repository layer acts as a layer between the business logic of the application and the data storage.

Repositories have only one concern: handling data access.

A user repository may have functions such as:
 - `create()`
 - `find_by_id`
 - `find_by_email`
 - `update()`
