FROM maven:3.8.5-openjdk-17 AS build
WORKDIR /app
COPY . .
RUN mvn clean package -DskipTests

FROM eclipse-temurin:17-jdk-alpine
WORKDIR /app
COPY --from=build /app/target/ordersservice-0.0.1-SNAPSHOT.jar app.jar

# puerto interno de spring boot
EXPOSE 8080
ENTRYPOINT ["java", "-jar", "app.jar"]