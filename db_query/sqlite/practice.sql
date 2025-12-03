SELECT u.name,
    u.country,
    SUM(amount)
FROM users u
    JOIN orders o ON u.user_id = o.user_id
GROUP BY u.name,
    u.country