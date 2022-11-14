// Known good C++ file
// upload with minicore / clock int 8MHz / chip avr m8 / using ArduinoISP 

void setup() {
  pinMode(5, OUTPUT);
}

void loop() {  
  delay(100);
  digitalWrite(5, LOW);
  delay(100);
  digitalWrite(5, HIGH);
  delay(100);
  digitalWrite(5, LOW);
  delay(500);
  digitalWrite(5, HIGH);
}