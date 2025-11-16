// To use, you must configure Arduino IDE's settings
// * tools->board: Arduino nano matter
// * tools->port: XXX
// * you MIGHT need to install "Silicon Labs" from "Boards Manager"


void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
  digitalWrite(LED_BUILTIN, HIGH);
  delay(1000);
  digitalWrite(LED_BUILTIN, LOW);
  delay(1000);
}
