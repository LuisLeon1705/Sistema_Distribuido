const express = require("express");

const router = express.Router();
const controller = require("../controllers/payment.controller");
const { verifyToken } = require("../middlewares/auth");

router.post("/", verifyToken, controller.processPayment);

router.get("/", verifyToken, controller.getPayments);

router.get("/:id", verifyToken, controller.getPaymentById);

module.exports = router;