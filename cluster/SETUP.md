
source kops.sh

# IAM setup

aws iam create-group --group-name kops

aws iam attach-group-policy --policy-arn arn:aws:iam::aws:policy/AmazonEC2FullAccess --group-name kops
aws iam attach-group-policy --policy-arn arn:aws:iam::aws:policy/AmazonRoute53FullAccess --group-name kops
aws iam attach-group-policy --policy-arn arn:aws:iam::aws:policy/AmazonS3FullAccess --group-name kops
aws iam attach-group-policy --policy-arn arn:aws:iam::aws:policy/IAMFullAccess --group-name kops
aws iam attach-group-policy --policy-arn arn:aws:iam::aws:policy/AmazonVPCFullAccess --group-name kops

aws iam create-user --user-name kops

aws iam add-user-to-group --user-name kops --group-name kops

aws iam create-access-key --user-name kops

# DNS setup

ID=$(uuidgen) && aws route53 create-hosted-zone --name k8s.zeta-two.com --caller-reference $ID | jq .DelegationSet.NameServers
dig ns k8s.zeta-two.com

# State storage setup

aws s3api create-bucket --bucket k8s.zeta-two.com --region eu-north-1 --create-bucket-configuration LocationConstraint=eu-north-1
aws s3api put-bucket-versioning --bucket k8s.zeta-two.com  --versioning-configuration Status=Enabled
aws s3api put-bucket-encryption --bucket k8s.zeta-two.com --server-side-encryption-configuration '{"Rules":[{"ApplyServerSideEncryptionByDefault":{"SSEAlgorithm":"AES256"}}]}'


# Create cluster

kops create cluster --zones eu-north-1a --master-size t3.micro --node-size t3.nano ${NAME}
kops create secret --name k8s.zeta-two.com sshpublickey admin -i ~/.ssh/ZetaTwo2018.pub
kops edit cluster ${NAME}
kops update cluster ${NAME} --yes

kubectl get nodes
kops validate cluster
