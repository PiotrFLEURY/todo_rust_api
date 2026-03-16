# Curl HTTP Requests collection

**Create a new todo:**
```bash
curl -X POST -d '{"title":"something to do"}' -H "Content-Type:application/json" http://localhost:8080/todos
```

**Get all todos:**
```bash
curl http://localhost:8080/todos
```

**Update a todo:**
```bash
curl -X PUT -d '{"completed":true}' -H "Content-Type:application/json" http://localhost:8080/todos/1
```

**Delete a todo:**
```bash
curl -X DELETE http://localhost:8080/todos/1
```