# StyleMixer: Cloud-based Neural Style Transfer Service

### Introduction

In this project, we build a cloud-based service for neural style transfer and deploy our service to the AWS AppRunner [[Link](https://ktthnbhkmf.us-east-1.awsapprunner.com/)]. The motivation of this project is based on one of our previously pubished paper "Style Mixer: Semantic-aware Multi-Style Transfer Network" [[Paper](https://arxiv.org/abs/1910.13093)] [[Github](https://github.com/zxhuang97/Official-implementation-StyleMixer)]. Specifically, our application enables users to enjoy powerful functions of style transfer to generate fancy images via operations as simple as only button-clicks.

### Background

Thanks to the development of computer vision and graphics technology, recent years have witnessed an increasing number of photo apps for image editing. Style Transfer, which takes a content image and a style image as inputs to synthesize a stylized image, has become more and more popular. Given a photo of women (upper left corner) and art works with different styles (bottom left corner), the algorithm can generate stylized versions of the same women without compromising its appearance. 

![img](./assets/1.png)

Typically, the script of style transfer requires complicated operations and involves deep neural network for image processing (figure below illustrates a standard pipeline of style transfer). However, this is not user-friendly to users without computer science background. To this end, we build this application and deploy it as a cloud-based service to allow all sorts of users to enjoy the charm of style transfer. Generally speaking, with our StyleMixer, you can upload your own image and choose a style you like as provided by our service. Then, you will be able to get a stylized version of your uploaded content image.

![img](./assets/2.png)

Here's an example of content image (a photo of duke!!!), style image (from Wikiart) and stylized image.