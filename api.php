<?php
$date = new DateTime();
$data = file_get_contents("php://input");

$file = fopen($date->getTimestamp() . rand(1,1000),"w");
fwrite($file, $data);
fclose($file);
?>