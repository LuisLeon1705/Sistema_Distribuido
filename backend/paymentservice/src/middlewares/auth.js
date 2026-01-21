const jwt = require("jsonwebtoken");

exports.verifyToken = (req, res, next) => {
  const token = req.headers["authorization"];

  if (!token) return res.status(403).json({ message: "Token required" });

  try {
    const bearer = token.split(" ")[1];
    const decoded = jwt.verify(bearer, process.env.JWT_SECRET);
    req.user = decoded;
    next();
  } catch (e) {
    return res.status(401).json({ message: "Invalid or expired token" });
  }
};