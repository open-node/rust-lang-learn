let times = 0;
while (true) {
  console.log("loop times: %s", (times += 1));
  if (times > 100000) break;
}
