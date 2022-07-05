# Ceph 分布式部署文档

## 部署环境

- VMware Workstation 16 Pro
- CentOS 7 64位（CentOS 7.9）
- IP 段：192.168.153.0（实验机从 192.168.153.128 开始）

## 部署过程

这里每台实验机都是从刚刚安装完 ceph 的第一台实验机（也就是 192.168.153.128）直接克隆得到的，这样可以避免重复而冗长的安装和配置环节。下面进行的是对克隆之后的实验机进行的额外配置。

### 配置主机名

```shell
### 在所有实验机上重新修改，这里以第 2 个为例

# 设置 hostname
hostnamectl set-hostname ceph2
 
# 配置 hosts 解析
echo "192.168.153.130 ceph2">>/etc/hosts
 
# 重启
reboot
```

### 配置管理主机

```shell
# 配置管理主机（第一个节点）ceph, 使之可以通过 SSH 无密码访问各节点
ssh-keygen
ssh-copy-id ceph2
ssh-copy-id ceph3
```

### 其他配置

- 根据[这篇文章](https://zhuanlan.zhihu.com/p/390377674)的提示，进行 NTP 的配置。

```shell
# 确保各节点的用户都有sudo权限
echo "{username} ALL = (root) NOPASSWD:ALL" | sudo tee /etc/sudoers.d/{username}
sudo chmod 0440 /etc/sudoers.d/{username}
```

### 部署操作

``` shell
# 在管理节点新建工作目录，后续操作在工作目录下完成
mkdir cluster; cd cluster

# 创建 monitor 节点
ceph-deploy new ceph

# 生成 monitor 节点检测集群所需要的密钥文件，将生成的ceph.client.admin.keyring和配置文件推送到各节点
ceph-deploy mon create-initial
sudo chmod +r /etc/ceph/ceph.client.admin.keyring
ceph-deploy admin ceph ceph2 ceph3

# 部署mgr节点
ceph-deploy mgr create ceph

# 部署osd节点
ceph-deploy osd create ceph --data /dev/sdb
ceph-deploy osd create ceph2 --data /dev/sdb
ceph-deploy osd create ceph3 --data /dev/sdb
```

到这里分布式部署就结束了。

