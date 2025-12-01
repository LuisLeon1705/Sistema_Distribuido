# Inventory Service API Endpoints

This document describes the REST API endpoints provided by the Inventory Service.

## Base URL

All endpoints are relative to the service's base URL (e.g., `http://localhost:8000`).

---

## 1. Order Endpoints

### `GET /orders`
Retrieves a list of all orders.
- **Method:** `GET`
- **URL:** `/orders`
- **Request Body:** None
- **Response:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
      "total_price": "123.45",
      "status": "pending",
      "created_at": "2023-10-27T10:00:00Z"
    }
  ]
  ```

### `POST /orders`
Creates a new order from a temporary order associated with a user.
- **Method:** `POST`
- **URL:** `/orders`
- **Request Body:**
  ```json
  {
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "items": []
  }
  ```
  *(Note: The `items` array in the request body is currently ignored. The actual order items are pulled from the temporary order associated with the `user_id`.)*
- **Response:** `201 Created`
  ```json
  {
    "id": 1,
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "total_price": "123.45",
    "status": "pending",
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
- **Error Responses:**
  - `400 Bad Request`: If no temporary order found for `user_id` or temporary order has no items.
  - `500 Internal Server Error`: For other server-side issues.

### `GET /orders/:order_id`
Retrieves a specific order by its ID.
- **Method:** `GET`
- **URL:** `/orders/{order_id}` (e.g., `/orders/1`)
- **Request Body:** None
- **Response:** `200 OK`
  ```json
  {
    "id": 1,
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "total_price": "123.45",
    "status": "pending",
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
- **Error Responses:**
  - `404 Not Found`: If the order with the given ID does not exist.
  - `500 Internal Server Error`: For other server-side issues.

### `GET /orders/:order_id/items`
Retrieves all items belonging to a specific order.
- **Method:** `GET`
- **URL:** `/orders/{order_id}/items` (e.g., `/orders/1/items`)
- **Request Body:** None
- **Response:** `200 OK`
  ```json
  [
    {
      "id": 101,
      "order_id": 1,
      "product_id": "c1d2e3f4-g5h6-7890-1234-567890abcdef",
      "quantity": 2,
      "price_at_time_of_purchase": "50.00"
    }
  ]
  ```
- **Error Responses:**
  - `500 Internal Server Error`: For server-side issues.

### `GET /orders/user/:user_id`
Retrieves all orders placed by a specific user.
- **Method:** `GET`
- **URL:** `/orders/user/{user_id}` (e.g., `/orders/user/a1b2c3d4-e5f6-7890-1234-567890abcdef`)
- **Request Body:** None
- **Response:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
      "total_price": "123.45",
      "status": "pending",
      "created_at": "2023-10-27T10:00:00Z"
    }
  ]
  ```
- **Error Responses:**
  - `500 Internal Server Error`: For server-side issues.

### `POST /orders/status`
Updates the status of an order. Only orders with a "pending" status can be updated. If the status is changed to "cancelled", the items are returned to stock.
- **Method:** `POST`
- **URL:** `/orders/status`
- **Request Body:**
  ```json
  {
    "order_id": 1,
    "new_status": "completed"
  }
  ```
  Possible `new_status` values: `"completed"`, `"cancelled"`.
- **Response:** `200 OK`
  ```json
  {
    "id": 1,
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "total_price": "123.45",
    "status": "completed",
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
- **Error Responses:**
  - `400 Bad Request`: If the order status is not "pending" or for invalid status transitions.
  - `404 Not Found`: If the order with the given ID does not exist.
  - `500 Internal Server Error`: For other server-side issues.

---

## 2. Stock Endpoints

### `POST /stock`
Adds new stock for a product.
- **Method:** `POST`
- **URL:** `/stock`
- **Request Body:**
  ```json
  {
    "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
    "quantity": 100,
    "warehouse_location": "Warehouse A"
  }
  ```
- **Response:** `200 OK`
  ```json
  {
    "id": 1,
    "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
    "quantity": 100,
    "last_updated": "2023-10-27T10:00:00Z",
    "warehouse_location": "Warehouse A"
  }
  ```
- **Error Responses:**
  - `500 Internal Server Error`: For server-side issues.

### `GET /stock`
Retrieves a list of all stock entries.
- **Method:** `GET`
- **URL:** `/stock`
- **Request Body:** None
- **Response:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
      "quantity": 100,
      "last_updated": "2023-10-27T10:00:00Z",
      "warehouse_location": "Warehouse A"
    }
  ]
  ```
- **Error Responses:**
  - `500 Internal Server Error`: For server-side issues.

### `GET /stock/:product_id`
Retrieves stock information for a specific product.
- **Method:** `GET`
- **URL:** `/stock/{product_id}` (e.g., `/stock/f1g2h3i4-j5k6-7890-1234-567890abcdef`)
- **Request Body:** None
- **Response:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
      "quantity": 100,
      "last_updated": "2023-10-27T10:00:00Z",
      "warehouse_location": "Warehouse A"
    }
  ]
  ```
- **Error Responses:**
  - `500 Internal Server Error`: For server-side issues.

### `PUT /stock/:product_id`
Updates the quantity or warehouse location of a specific product's stock.
- **Method:** `PUT`
- **URL:** `/stock/{product_id}` (e.g., `/stock/f1g2h3i4-j5k6-7890-1234-567890abcdef`)
- **Request Body:**
  ```json
  {
    "quantity": 120,
    "warehouse_location": "Warehouse B"
  }
  ```
  (Fields are optional; only provided fields will be updated)
- **Response:** `200 OK`
  ```json
  {
    "id": 1,
    "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
    "quantity": 120,
    "last_updated": "2023-10-27T10:00:00Z",
    "warehouse_location": "Warehouse B"
  }
  ```
- **Error Responses:**
  - `404 Not Found`: If the stock entry for the given product ID does not exist.
  - `500 Internal Server Error`: For other server-side issues.

### `DELETE /stock/:product_id`
Deletes a stock entry for a specific product.
- **Method:** `DELETE`
- **URL:** `/stock/{product_id}` (e.g., `/stock/f1g2h3i4-j5k6-7890-1234-567890abcdef`)
- **Request Body:** None
- **Response:** `204 No Content`
- **Error Responses:**
  - `404 Not Found`: If the stock entry for the given product ID does not exist.
  - `500 Internal Server Error`: For other server-side issues.

---

## 3. Temporary Order Endpoints

Temporary orders are stored in a local JSON file (`data/order_list.json`) and are used to hold order information before it is finalized and moved to the main database.

### `POST /temp_orders`
Adds a new temporary order. If a temporary order with the same `user_id` already exists, it will be replaced.
- **Method:** `POST`
- **URL:** `/temp_orders`
- **Request Body:**
  ```json
  {
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "items": [
      {
        "product_id": "b2c3d4e5-f6a7-8901-2345-67890abcdef1",
        "quantity": 1,
        "price": "10.00"
      },
      {
        "product_id": "c3d4e5f6-a7b8-9012-3456-7890abcdef21",
        "quantity": 3,
        "price": "5.50"
      }
    ]
  }
  ```
- **Response:** `201 Created`
  ```json
  {
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "items": [
      {
        "product_id": "b2c3d4e5-f6a7-8901-2345-67890abcdef1",
        "quantity": 1,
        "price": "10.00"
      }
    ],
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
  *(Note: The `quantity` in the response might be adjusted if it exceeds available stock.)*
- **Error Responses:**
  - `500 Internal Server Error`: For server-side issues.

### `GET /temp_orders/user/:user_id`
Retrieves all temporary orders associated with a specific user.
- **Method:** `GET`
- **URL:** `/temp_orders/user/{user_id}` (e.g., `/temp_orders/user/a1b2c3d4-e5f6-7890-1234-567890abcdef`)
- **Request Body:** None
- **Response:** `200 OK`
  ```json
  [
    {
      "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
      "items": [
        {
          "product_id": "b2c3d4e5-f6a7-8901-2345-67890abcdef1",
          "quantity": 1,
          "price": "10.00"
        }
      ],
      "created_at": "2023-10-27T10:00:00Z"
    }
  ]
  ```
- **Error Responses:**
  - `500 Internal Server Error`: For server-side issues.
