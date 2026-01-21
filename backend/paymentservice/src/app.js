const express = require("express");
const cors = require("cors");
const sequelize = require("./config/database");
const paymentRoutes = require("./routes/payment.routes");

const app = express();

app.use(cors());
app.use(express.json());

app.use("/payments", paymentRoutes);

const PORT = process.env.PORT || 3005;

sequelize.sync({ force: false }).then(() => {
  console.log("Database connected");
  app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
  });
}).catch(e => console.error("Database connection error:", e));