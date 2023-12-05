#include<bits/stdc++.h>
using namespace std;

typedef vector<int> vi; 
typedef pair<int, int> ii;
typedef vector<ii> vii;


vi dijkstra(int s, vector<vii> adj_list){
  int n = adj_list.size();
  vi distance(n, INT_MAX/2);
  
  //distance from s to s is zero...
  distance[s] = 0;
  priority_queue<ii, vii, greater<ii>> pq;
  pq.push(ii(s,0));

  while (!pq.empty()){
    auto[v, d] = pq.top();pq.pop();
   
    // avoid double checking
    if(distance[v] < d){
      continue;
    }
    
    //for every adjencecy of v, check if exists a path
    // from s to u through v such that |s ~> v ~> u| < |s -> u|
    for(auto [u, w]: adj_list[s]){
        if(distance[v] + w < distance[u]){
          distance[u] = distance[v] + w;
          pq.push(ii(u, distance[u]));
      }
    }
  }
  return distance;
}
