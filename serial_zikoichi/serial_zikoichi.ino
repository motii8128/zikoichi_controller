#include "zikoichi_kun_api.hpp"

#include <ArduinoJson.h>

JsonDocument doc;
char buf[256];

void setup() {
  Serial.begin(115200);
  zk_api::setup();
}

void loop() {
  int index = 0;
  while (Serial.available() > 0) {
    buf[index] = Serial.read();
    index++;
    //バッファ以上の場合は中断
    if (index >= 256) {
      break;
    }
  }

  DeserializationError err = deserializeJson(doc, buf);
  float fl = doc["front_left"];
  float fr = doc["front_right"];
  float rl = doc["rear_left"];
  float rr = doc["rear_right"];

  int fl_pow = abs_map(fl, 0.0, 1.0, 0, 255);
  int fr_pow = abs_map(fr, 0.0, 1.0, 0, 255);
  int rl_pow = abs_map(rl, 0.0, 1.0, 0, 255);
  int rr_pow = abs_map(rr, 0.0, 1.0, 0, 255);

  zk_api::control_front_left(fl_pow);
  zk_api::control_front_right(fr_pow);
  zk_api::control_rear_left(rl_pow);
  zk_api::control_rear_right(rr_pow);
}

int abs_map(float value, float in_min, float in_max, int out_min, int out_max)
{
  if(value > 0)
  {
    return (int)((value - in_min) * (float)(out_max - out_min) / (in_max - in_min) + (float)(out_min));
  }
  else
  {
    return -1 * (int)((std::abs(value) - in_min) * (float)(out_max - out_min) / (in_max - in_min) + (float)(out_min));
  }
}