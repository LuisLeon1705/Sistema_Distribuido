import smtplib, ssl
from email.message import EmailMessage

msg = EmailMessage()
msg["Subject"] = "Prueba SMTP"
msg["From"] = "servicio@linkstart.solutions"
msg["To"] = "eduar06tae@gmail.com"
msg.set_content("Este es un correo de prueba.")

context = ssl.create_default_context()
with smtplib.SMTP_SSL("mail.linkstart.solutions", 465, context=context) as server:
    server.login("servicio@linkstart.solutions", "New1234$$..")
    server.send_message(msg)

print("Correo enviado correctamente")
