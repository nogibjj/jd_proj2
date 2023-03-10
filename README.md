# Project 2 

## Overview
My project 2 is a microservice that finds concerts in the Durham area from Ticketmaster, aimed to help Duke students (or those in the RTP area) find concerts of their favorite artists. It also uses the Spotify recommendation algorithm to recommend similar artists for the user to check out if a given artist has no concerts scheduled in Durham. 

## Architecture 

<img width="800" alt="image" src="https://user-images.githubusercontent.com/86393045/221381284-d2c08000-be4a-4e01-9192-4baf9946d4a6.png">      


## Endpoints 
1. `/`: Welcome page

    <img width="550" alt="image" src="https://user-images.githubusercontent.com/86393045/221379875-114d32cc-8528-4c6d-be7c-cf34a145b48f.png">


2. `/artist/{artist_name}`: Uses artist name to search for concerts - returns either events found on Ticketmaster, or related artists if no events are found 

    For example, if I want to find Bruce Springsteen concerts, I would use path `/artist/bruce springsteen`. In this case, there is a concert in the area, and the microservice returns details:

   <img width="900" alt="image" src="https://user-images.githubusercontent.com/86393045/221379688-d47bd4e8-5adf-47ca-96ea-bab86abdbf89.png">


    Another example is if I'm curious if Adele is coming to Durham, i.e. `/artist/adele`. Here, Adele has  no concerts, and so a list of related artists are provided for the user to check out (and hopefully find another artist that they can search for concerts):

    <img width="750" alt="image" src="https://user-images.githubusercontent.com/86393045/221379862-ea5fb1f6-0d3d-4364-bcb2-b7f20a75bbd2.png">


    And if I accidentally spell her name wrong, i.e. `/artist/adelle`, it will return to try again: 
    
    <img width="600" alt="image" src="https://user-images.githubusercontent.com/86393045/221379674-5bca3456-bb2d-4e01-bf87-f091c7b5dad6.png">

    


## Setup 

### API Access
- Get key for Ticketmaster API
    - [Register for dev account](https://developer-acct.ticketmaster.com/user/register), which will automatically provide you with an API key 
- Get client id and key for Spotify API
    - [Register for dev account](https://developer.spotify.com/dashboard/#)
    - Go to dashboard and create an app, which will automatically give you Client ID and Client Secret. Find more details [here](https://developer.spotify.com/documentation/general/guides/authorization/app-settings/). 
- Set Secrets in Repo
    Secrets are stored using the `secretstore` crate. To set up the keys from above, please follow the following steps. 
    1. Install `securestore` and helper `sslclient` crate 
        - `cargo install securestore`
        - `cargo install sslclient`
    2. Create secrets key and json to store
        - `create secrets.json --export-key secrets.key`
    3. Store keys 
        - Ticketmaster:
            - `ssclient -k secrets.key set api_key {ticketmaster_api_key}`
        - Spotify 
            - `ssclient -k secrets.key set client_id {spotify_client_id}`
            - `ssclient -k secrets.key set client_secret {spotify_client_secret}`



### Running microservice
1. Running locally
    - install rust 
    - `cargo run` after `cd` into `jd_proj2`


2. Running on Kubernetes

    A repository for this app is created on DockerHub: [`jacdu/ticketmaster-concert`](https://hub.docker.com/repository/docker/jacdu/ticketmaster-concert/general). The instructions are adapted from [here](https://github.com/nogibjj/coursera-applied-de-kubernetes-lab).

    - install minikube 
    - Run the following commands: 
        1. `minikube start`
        2. Create a deployment `kubectl create deployment ticketmaster image=registry.hub.docker.com/jacdu/ticketmaster`
        3. View deployment: `kubectl get deployments`
        4. Create service and expose it: `kubectl expose deployment ticketmaster --type=LoadBalancer --port=8080`
        5. View services: `kubectl get service ticketmaster-concert:first-image-push`
        6. `minikube service ticketmaster --url`
        7. Curl web service: i.e. `curl http://192.168.49.2:30082`
        8. Clean up 
            - `kubectl delete service ticketmaster`
            - `kubectl delete deployment ticketmaster`
            - `minikube stop`
    


## Benchmarking
The below shows the timing for the different possibilities in the `/artist/{artist_name}` path. The base path is not shown as it is a trivial case. 

<img width="400" alt="image" src="https://user-images.githubusercontent.com/86393045/221379680-1ac4e746-1c41-4f25-b640-d00dc24d3386.png">


## References
- [Minikube Tutorial](https://minikube.sigs.k8s.io/docs/start/)
- [Spotify Developer API Docs](https://developer.spotify.com/documentation/web-api/)
- [Ticketmaster API Docs](https://developer.ticketmaster.com/products-and-docs/apis/discovery-api/v2/#search-events-v2)
