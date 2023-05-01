# An Image Style Transfer Web Service

## Architecure
The solution will set up a CodePipeline that pulls the code from GitHub and builds the codes, then stores the container image artifact in Amazon ECR. App Runner is configured to trigger automatic deployments once a new image is pushed to ECR. Python applications running in App Runner will stream the log to Amazon CloudWatch.

<img src="https://user-images.githubusercontent.com/33047941/235515401-d226f7f3-6cbb-4a96-aed7-e1dfac4757b1.png" alt="alt text" width="70%" height="70%">
  

## Implements

- Frontend
    - We employed ```Booststrapt```  to construct the responsive layout, and utilized ```Ajax``` to render the progress bar and output asynchronously.

- Backend
    - We implemented two versions of this application, one is in Rust and the other one is in Python.
    - For the Rust version, we used the ```actix Web``` framework to handle the frontend request in a asynchromous way and respond the frontend request with JSON data.
    - For the Python version, we used ```Flask``` to handle the frontend requests.

- This application can read the user uploaded images at frontend and query the progress and output at backend.

## Deployment
- Create ECR and build docker image in Cloud9 environment.

    ![image](https://user-images.githubusercontent.com/33047941/235516384-1cb16660-a2fe-450a-be12-66684ee682b6.png)

- Push the latest image to ECR

    <img src="https://user-images.githubusercontent.com/33047941/235516543-e3235fbd-e380-4317-80d1-a579ca9d1f9a.png" alt="alt text" width="70%" height="70%">
    

- Run the service using APP runner

    <img src="https://user-images.githubusercontent.com/33047941/235516935-8f37316e-d14b-4611-9c8f-243da0184b43.png" alt="alt text" width="70%" height="70%">


## Usage
- Please use **High Resolution images** or it may block our API
- It may take some times to render the output **DON'T REFRESH THE PAGE** while waiting

![21f4b46bc76d3b3710434e81a4d3969](https://user-images.githubusercontent.com/33047941/235515932-28ca208b-9dfe-4164-8688-bb35d1b041db.png)

![9101ea9542c03cd6b10f15a46182a70](https://user-images.githubusercontent.com/33047941/235516158-3f79bc73-f18f-43d7-b7a4-3ca5d63b5e9f.jpg)


![27db7a6d8dea9f1d9cfcfaf2022283b](https://user-images.githubusercontent.com/33047941/235516081-f3e49077-7a11-441c-a645-75e808aec928.png)


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
