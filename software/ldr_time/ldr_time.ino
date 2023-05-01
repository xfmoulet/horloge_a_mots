// LDR time reading correponding to /doc/clock.html, proof of concept for arduino

// the setup routine runs once when you press reset:
void setup() {
  // initialize serial communication at 9600 bits per second:
  Serial.begin(9600);
}
 
// the loop routine runs over and over again forever:
int oldvalue = 0;
int nb = 0;
bool bits[14];
uint8_t bitpos = 0;
 
void loop() {
 
  // read the input on analog pin 0:
  int sensorValue = analogRead(A7);
  int value = sensorValue > 500;
 
  if (oldvalue == value) {  
    nb +=1;
  } else {
    // swapped, we received something
    Serial.print(nb);
    Serial.print(" ");
 
    if (nb<50) {
      Serial.println("too short");      
    } else {
      if (nb > 800) {
        Serial.println("reset ======= ");
        if (bitpos!=13) {
          Serial.println("Didn't receive 13 bits, avoiding");
        } else {
            uint8_t parity =0;
            for (int i=0;i<13-1;i++) {
               parity += bits[i];
            }
            if (bits[12] != parity%2) {
              Serial.println("parity error, avoiding");
            } else {
              uint8_t hour = bits[0]*16+bits[1]*8+bits[2]*4+bits[3]*2+bits[4];
              uint8_t min5 = bits[5]*8+bits[6]*4+bits[7]*2+bits[8];
              uint8_t minute = bits[9]*4+bits[10]*2+bits[12];
 
              Serial.print("Got an hour! it's: ");
              Serial.print(hour);
              Serial.print(":");
              Serial.print(min5*5+minute);
              Serial.println();
              for (int i=0;i<13;i++) {
                Serial.print(bits[i]);
              }
 
              Serial.println();
            }
        }     
        bitpos = 0;
      } else if (oldvalue==1) { // length of high level defines the next bit
          if (nb>120) {
            Serial.println("long : 1");
            bits[bitpos] = 1;
            bitpos +=1;
          } else {
            Serial.println("short: 0");
            bits[bitpos] = 0;
            bitpos +=1;
          }
      } 
    }
 
 
    nb = 0;
    oldvalue = value;
  }
 
  delay(1);        // delay in between reads for stability
}