package scheduler

import (
	"github.com/buraksezer/consistent"
	"github.com/cespare/xxhash"
	"log"
)

// consistent package doesn't provide a default hashing function.
// You should provide a proper one to distribute keys/members uniformly.
type hasher struct{}

func (h hasher) Sum64(data []byte) uint64 {
	// you should use a proper hash function for uniformity.
	return xxhash.Sum64(data)
}

type State struct {
	Members []consistent.Member
	CurrentSize int
	MaxConcurrency int
}

type Scheduler struct {
	state State
	hashRing *consistent.Consistent
}

func CreateScheduler(state State) *Scheduler {
	config := consistent.Config{
		Hasher:            hasher{},
		PartitionCount:    271,
		ReplicationFactor: 20,
		Load:              1.25,
	}

	return &Scheduler{
		state: state,
		hashRing: consistent.New(state.Members, config),
	}
}

// Scale changes the size of Discord shards
func (s *Scheduler) Scale(newScale int) {
	log.Printf("scaling from %d shards to %d shards", s.state.CurrentSize, newScale)
	// todo: take the scale mutex

	// we should handle all the sessions restart
	// and the re-scheduling of the shards including rate-limiting
	// first, we remove all the shards instances from the clusters
	for i := 0; i < s.state.CurrentSize; i++ {
		// todo: call the rpc for stopping the shard on the given cluster
		cluster := s.hashRing.LocateKey([]byte{byte(i)})
		log.Printf("sending stop shard to shard %d on cluster %s", i, cluster)
	}
	s.state.CurrentSize = newScale


	// after we stopped the shard, we re-start it with the new number of shards
	// since all the shard must be restarted after a scale operation
	// but before that, we need to allocate all the shards
	for i := 0; i < s.state.CurrentSize; i++ {
		cluster := s.hashRing.LocateKey([]byte{byte(i)})
		// todo: call the rpc for allocate the shard on the given cluster
		log.Printf("sending allocate shard to shard %d on %s with [%d, %d] as sharding key", i, cluster, i, newScale)
	}

	// we start the first shard, this represents the "pilot" shard which gives us the
	// required information, such as the max_concurrency.
	// todo: call the rpc to start the first shard

	// according to the max_concurrency system, we must
	// start the shards sequentially.
	for i := 0; i < s.state.MaxConcurrency; i++ {
		log.Printf("starting bucket %d", i)
		for j := 0; j < newScale; j++ {
			// if the shard corresponds to the current starting bucket
			if j % s.state.MaxConcurrency == i {
				cluster := s.hashRing.LocateKey([]byte{byte(j)})
				// todo: call the start rpc on the given cluster
				log.Printf("%d: sending the start command to the shard %d on the %s cluster", i, j, cluster)
			}
		}
	}

	// todo: release the scale mutex
}

// RemoveNode removes a cluster from the consistent hash ring and relocate the shards
func (s *Scheduler) RemoveNode() {

}

// AddNode adds a node to the consistent hash ring and relocate the shards
func (s *Scheduler) AddNode() {}

// GetShardLocation returns the supposed name of the cluster containing a Shard
func (s *Scheduler) GetShardLocation() string {
	return ""
}
