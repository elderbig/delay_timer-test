#please run this monit script in tester dir
pid=`ps -ef|grep dt_test|grep -v grep|awk '{print $2}'`
#clean last test result
>mem.log
while true;do t=`date "+%Y-%m-%d %H:%M:%S"`&&m=`ps o rss -p $pid|tail -1`;echo "$t | $m";sleep 5;done>>mem.log