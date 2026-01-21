const Payment = require("../models/payment");

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

    const payment = await Payment.create({
      orderId,
      amount,
      currency,
      status: "PENDING",
    });

    setTimeout(async () => {
      const isSuccess = Math.random() > 0.2; 
      
      const finalStatus = isSuccess ? "COMPLETED" : "FAILED";
      const mockTransaction = `tx_${Math.floor(Math.random() * 1000000)}`;

      await payment.update({ 
        status: finalStatus, 
        transactionId: mockTransaction 
      });
      
      console.log(`Payment processed for Order ${orderId}: ${finalStatus}`);
      
      // TODO: Send webhook to Order service, to notify payment status 
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