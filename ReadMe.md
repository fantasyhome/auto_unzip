## 只支持Bandizip,想要其他解压软件可以联系我提交Merge Request

# 使用方式
修改config.ini中的bz_dir = D:\tool\Bandizip\bz.exe路径
为自己的Bandizip目录下bz.exe程序的路径，没有此文件请升级Bandizip版本。

# 已知问题
此程序使用了三方库infer，无法判断.ts视频类型，issue如下
https://github.com/bojand/infer/issues/111

当前解决方法：此情况temp_extract目录不会删除，需手动在生成目录下寻找视频文件。