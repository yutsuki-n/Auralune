SELECT user_id,
    Count(*) AS cnt
From orders
GROUP BY user_id;
SELECT country,
    Count(*) AS cnt
From users
GROUP BY country;
SELECT u.country,
    SUM(o.amount) AS total_sales
FROM users u
    JOIN orders o ON o.user_id = u.user_id
GROUP BY u.country
ORDER BY total_sales DESC