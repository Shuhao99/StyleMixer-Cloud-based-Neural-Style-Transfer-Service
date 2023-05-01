# StyleMixer: Cloud-based Neural Style Transfer Service

### Introduction

In this project, we build a cloud-based service for neural style transfer and deploy our service to the AWS AppRunner [[Link](https://ktthnbhkmf.us-east-1.awsapprunner.com/)]. The motivation of this project is based on one of our previously pubished paper "Style Mixer: Semantic-aware Multi-Style Transfer Network" [[Paper](https://arxiv.org/abs/1910.13093)] [[Github](https://github.com/zxhuang97/Official-implementation-StyleMixer)]. Specifically, our application enables users to enjoy powerful functions of style transfer to generate fancy images via operations as simple as only button-clicks.

### Background

Thanks to the development of computer vision and graphics technology, recent years have witnessed an increasing number of photo apps for image editing. Style Transfer, which takes a content image and a style image as inputs to synthesize a stylized image, has become more and more popular. Given a photo of women (upper left corner) and art works with different styles (bottom left corner), the algorithm can generate stylized versions of the same women without compromising its appearance. 

![img](./assets/1.png)

Typically, the script of style transfer requires complicated operations and involves deep neural network for image processing (figure below illustrates a standard pipeline of style transfer). However, this is not user-friendly to users without computer science background. To this end, we build this application and deploy it as a cloud-based service to allow all sorts of users to enjoy the charm of style transfer. Generally speaking, with our StyleMixer, you can upload your own image and choose a style you like as provided by our service. Then, you will be able to get a stylized version of your uploaded content image.

![img](./assets/2.png)

Here's an example of content image (a photo of duke!!!), style image (from Wikiart) and stylized image.


## Architecure
  ![Blank diagram](https://user-images.githubusercontent.com/33047941/235515401-d226f7f3-6cbb-4a96-aed7-e1dfac4757b1.png)

## Implements
  - Frontend
    - We use 
## Deployment
- Create ECR and build docker image in Cloud9 environment.
![image](https://user-images.githubusercontent.com/33047941/235516384-1cb16660-a2fe-450a-be12-66684ee682b6.png)
- Push the latest image to ECR
![image](https://user-images.githubusercontent.com/33047941/235516543-e3235fbd-e380-4317-80d1-a579ca9d1f9a.png)

- Run the service using APP runner
![image](https://user-images.githubusercontent.com/33047941/235516935-8f37316e-d14b-4611-9c8f-243da0184b43.png)


## Usage
- Please use High Resolution images or it may block our API
- It may take some times to render the output, DON'T REFRESH THE PAGE while waiting
![21f4b46bc76d3b3710434e81a4d3969](https://user-images.githubusercontent.com/33047941/235515932-28ca208b-9dfe-4164-8688-bb35d1b041db.png)
![9101ea9542c03cd6b10f15a46182a70](https://user-images.githubusercontent.com/33047941/235516158-3f79bc73-f18f-43d7-b7a4-3ca5d63b5e9f.jpg)


![27db7a6d8dea9f1d9cfcfaf2022283b](https://user-images.githubusercontent.com/33047941/235516081-f3e49077-7a11-441c-a645-75e808aec928.png)


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
