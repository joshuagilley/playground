package main

import (
	"fmt"
	"log"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/ec2"
)

func main() {
	// Start a session with the default AWS config
	sess, err := session.NewSession(&aws.Config{
		Region: aws.String("us-east-1"), // Replace with your region
	})
	if err != nil {
		log.Fatalf("failed to create session, %v", err)
	}

	// Create an EC2 service client
	svc := ec2.New(sess)

	// Example of creating a new EC2 instance (t2.micro)
	runResult, err := svc.RunInstances(&ec2.RunInstancesInput{
		ImageId:      aws.String("ami-00a929b66ed6e0de6"), // Replace with a valid AMI ID in your region
		InstanceType: aws.String("t2.micro"),            // Instance type (you can change this)
		MinCount:     aws.Int64(1),
		MaxCount:     aws.Int64(1),
	})
	if err != nil {
		log.Fatalf("failed to launch instance, %v", err)
	}

	fmt.Printf("Successfully launched EC2 instance: %s\n", *runResult.Instances[0].InstanceId)
}
