terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.27"
    }
  }

  required_version = ">= 0.14.9"
}

provider "aws" {
  profile = "default"
  region  = "us-west-2"
}

//resource "aws_instance" "app_server" {
//  ami           = "ami-0800fc0fa715fdcfe"
//  instance_type = "t2.micro"
//
//  tags = {
//    Name = "contrabanned"
//  }
//}

resource "aws_iam_role" "contrabanned-role" {
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      },
    ]
  })

  name                = "contrabanned-role"
  managed_policy_arns = [aws_iam_policy.server-policy.arn]
}

resource "aws_iam_policy" "server-policy" {
  name = "contrabanned-policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action   = ["dynamodb:*"]
        Effect   = "Allow"
        Resource = "arn:aws:dynamodb:us-west-2:161051971148:table/RedirectSets"
      },
    ]
  })
}