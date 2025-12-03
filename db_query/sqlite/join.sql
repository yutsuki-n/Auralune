SELECT o.order_date,
    u.name,
    o.amount
FROM orders o
    JOIN users u ON o.user_id = u.user_id;
SELECT o.order_id,
    o.amount
FROM orders o
    JOIN users u ON o.user_id = u.user_id
WHERE u.name = 'Bob';
SELECT *
FROM products