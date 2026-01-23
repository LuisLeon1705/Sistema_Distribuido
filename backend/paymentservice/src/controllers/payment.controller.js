const Payment = require("../models/payment");
const axios = require("axios");

exports.getPayments = async (req, res) => {
  try {
    const whereClause = req.query.orderId ? { orderId: req.query.orderId } : {};
    const payments = await Payment.findAll({ where: whereClause });
    res.json(payments);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};

exports.getPaymentById = async (req, res) => {
  try {
    const payment = await Payment.findByPk(req.params.id);
    if (!payment) return res.status(404).json({ message: "Payment not found" });
    res.json(payment);
  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};

exports.processPayment = async (req, res) => {
  try {
    const { orderId, amount, currency } = req.body;
    const authToken = req.headers["authorization"];

    const payment = await Payment.create({
      orderId,
      amount,
      currency,
      status: "PENDING",
    });

    // Notify Order Service: Payment Pending
    try {
        await axios.put(
            `http://ordersservice:8080/api/orders/${orderId}/status`,
            { status: "PENDING" },
            { headers: { Authorization: authToken } }
        );
        console.log(`Order ${orderId} status updated to PENDING`);
    } catch (error) {
        console.error(`Failed to update order ${orderId} status to PENDING:`, error.message);
    }

    setTimeout(async () => {
      const isSuccess = Math.random() > 0.2; 
      
      const finalStatus = isSuccess ? "COMPLETED" : "FAILED";
      const mockTransaction = `tx_${Math.floor(Math.random() * 1000000)}`;

      await payment.update({ 
        status: finalStatus, 
        transactionId: mockTransaction 
      });
      
      console.log(`Payment processed for Order ${orderId}: ${finalStatus}`);
      
      // Notify Order Service: Payment Completed or Failed
      try {
          await axios.put(
              `http://ordersservice:8080/api/orders/${orderId}/status`,
              { status: finalStatus },
              { headers: { Authorization: authToken } }
          );
          console.log(`Order ${orderId} status updated to ${finalStatus}`);
      } catch (error) {
          console.error(`Failed to update order ${orderId} status to ${finalStatus}:`, error.message);
      }
      
    }, 5000);

    res.status(201).json({
      message: "Payment started, processing...",
      paymentId: payment.id,
      status: "PENDING"
    });

  } catch (e) {
    res.status(500).json({ error: e.message });
  }
};