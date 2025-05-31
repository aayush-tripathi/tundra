<?php
$n = 80;
$a = 0;
$b = 1;
for ($i = 0; $i < $n; $i++) {
  [$a, $b] = [$b, $a + $b];
}
echo $a, PHP_EOL;