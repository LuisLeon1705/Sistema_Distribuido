const { DataTypes } = require("sequelize");
const sequelize = require("../config/database");

const Payment = sequelize.define("Payment", {
  id: {
    type: DataTypes.UUID,
    defaultValue: DataTypes.UUIDV4,
    primaryKey: true,
  },
  orderId: {
    type: DataTypes.STRING,
    allowNull: false,
  },
  amount: {
    type: DataTypes.DECIMAL(10, 2),
    allowNull: false,
  },
  currency: {
    type: DataTypes.STRING,
    defaultValue: "USD",
  },
  status: {
    type: DataTypes.ENUM("PENDING", "COMPLETED", "FAILED", "DECLINED"),
    defaultValue: "PENDING",
  },
  transactionId: {
    type: DataTypes.STRING,
  }
});

module.exports = Payment;