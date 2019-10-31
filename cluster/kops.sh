#!/bin/sh

export AWS_DEFAULT_PROFILE=kops

export AWS_ACCESS_KEY_ID=$(aws configure get aws_access_key_id)
export AWS_SECRET_ACCESS_KEY=$(aws configure get aws_secret_access_key)

export NAME=k8s.zeta-two.com
export KOPS_STATE_STORE=s3://k8s.zeta-two.com

#kops export kubecfg k8s.zeta-two.com # If setting up on new machine
