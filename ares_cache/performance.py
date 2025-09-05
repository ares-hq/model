"""
Performance utilities for ARES cache optimization.
"""

from typing import Dict, List, Any, TypeVar, Callable, Optional, TYPE_CHECKING
from functools import wraps, lru_cache
import time
from dataclasses import dataclass
from collections import defaultdict

if TYPE_CHECKING:
    from .team import Team
    from .season import Season

T = TypeVar('T')

@dataclass
class CacheStats:
    """Statistics for cache performance monitoring."""
    hits: int = 0
    misses: int = 0
    cache_size: int = 0
    
    @property
    def hit_rate(self) -> float:
        """Calculate cache hit rate."""
        total = self.hits + self.misses
        return self.hits / total if total > 0 else 0.0
    
    def __str__(self) -> str:
        return f"Cache Stats: {self.hits} hits, {self.misses} misses, {self.hit_rate:.2%} hit rate"


class PerformanceMonitor:
    """Monitor performance of cache operations."""
    
    def __init__(self):
        self.operation_times: Dict[str, List[float]] = defaultdict(list)
        self.cache_stats: Dict[str, CacheStats] = defaultdict(CacheStats)
    
    def time_operation(self, operation_name: str):
        """Decorator to time operations."""
        def decorator(func: Callable[..., T]) -> Callable[..., T]:
            @wraps(func)
            def wrapper(*args, **kwargs) -> T:
                start_time = time.perf_counter()
                try:
                    result = func(*args, **kwargs)
                    return result
                finally:
                    end_time = time.perf_counter()
                    duration = end_time - start_time
                    self.operation_times[operation_name].append(duration)
            return wrapper
        return decorator
    
    def record_cache_hit(self, cache_name: str):
        """Record a cache hit."""
        self.cache_stats[cache_name].hits += 1
    
    def record_cache_miss(self, cache_name: str):
        """Record a cache miss."""
        self.cache_stats[cache_name].misses += 1
    
    def get_operation_stats(self, operation_name: str) -> Dict[str, float]:
        """Get statistics for a specific operation."""
        times = self.operation_times.get(operation_name, [])
        if not times:
            return {}
        
        return {
            "count": len(times),
            "total_time": sum(times),
            "avg_time": sum(times) / len(times),
            "min_time": min(times),
            "max_time": max(times)
        }
    
    def get_all_stats(self) -> Dict[str, Any]:
        """Get all performance statistics."""
        operation_stats = {
            name: self.get_operation_stats(name) 
            for name in self.operation_times.keys()
        }
        
        cache_stats = {
            name: {
                "hits": stats.hits,
                "misses": stats.misses,
                "hit_rate": stats.hit_rate
            }
            for name, stats in self.cache_stats.items()
        }
        
        return {
            "operations": operation_stats,
            "caches": cache_stats
        }
    
    def reset_stats(self):
        """Reset all performance statistics."""
        self.operation_times.clear()
        self.cache_stats.clear()


class BatchProcessor:
    """Utilities for batch processing operations efficiently."""
    
    @staticmethod
    def batch_update_teams(teams: List['Team'], updates: Dict[int, Dict[str, Any]]) -> None:
        """Batch update multiple teams efficiently."""
        from .team import Team
        
        for team in teams:
            if team.number in updates:
                team_updates = updates[team.number]
                
                # Update scores if provided
                if 'auto' in team_updates:
                    object.__setattr__(team, 'auto', team_updates['auto'])
                if 'teleop' in team_updates:
                    object.__setattr__(team, 'teleop', team_updates['teleop'])
                if 'endgame' in team_updates:
                    object.__setattr__(team, 'endgame', team_updates['endgame'])
                if 'penalties' in team_updates:
                    object.__setattr__(team, 'penalties', team_updates['penalties'])
                
                # Update metadata if provided
                if 'lastMatch' in team_updates:
                    object.__setattr__(team, 'lastMatch', team_updates['lastMatch'])
                if 'lastChecked' in team_updates:
                    object.__setattr__(team, 'lastChecked', team_updates['lastChecked'])
                
                # Invalidate cache after all updates
                team.invalidate_cache()
    
    @staticmethod
    def batch_create_teams(team_data: List[Dict[str, Any]]) -> List['Team']:
        """Create multiple teams efficiently."""
        from .team import Team
        from .scoring import Auto, Teleop, Endgame, Penalties
        
        teams = []
        for data in team_data:
            # Extract scoring data
            auto = data.get('auto', Auto())
            teleop = data.get('teleop', Teleop())
            endgame = data.get('endgame', Endgame())
            penalties = data.get('penalties', Penalties())
            
            team = Team(
                number=data['number'],
                name=data.get('name', ''),
                sponsors=data.get('sponsors', ''),
                location=data.get('location', ''),
                lastMatch=data['lastMatch'],
                lastChecked=data['lastChecked'],
                auto=auto,
                teleop=teleop,
                endgame=endgame,
                penalties=penalties
            )
            teams.append(team)
        
        return teams
    
    @staticmethod
    def bulk_rank_calculation(teams: List['Team'], category: str = 'overall') -> None:
        """Calculate rankings for all teams in bulk."""
        if not teams:
            return
        
        # Get the appropriate scoring attribute
        if category == 'overall':
            sorted_teams = sorted(teams, key=lambda t: t.overall.score, reverse=True)
        elif category == 'auto':
            sorted_teams = sorted(teams, key=lambda t: t.auto.score, reverse=True)
        elif category == 'teleop':
            sorted_teams = sorted(teams, key=lambda t: t.teleop.score, reverse=True)
        elif category == 'endgame':
            sorted_teams = sorted(teams, key=lambda t: t.endgame.score, reverse=True)
        elif category == 'penalties':
            sorted_teams = sorted(teams, key=lambda t: t.penalties.score, reverse=False)  # Lower is better
        else:
            raise ValueError(f"Invalid category: {category}")
        
        # Assign ranks
        for rank, team in enumerate(sorted_teams, 1):
            if category == 'overall':
                team.overall.rank = rank
            elif category == 'auto':
                team.auto.rank = rank
            elif category == 'teleop':
                team.teleop.rank = rank
            elif category == 'endgame':
                team.endgame.rank = rank
            elif category == 'penalties':
                team.penalties.rank = rank
            
            # Invalidate cache to reflect ranking changes
            team.invalidate_cache()


class MemoryOptimizer:
    """Utilities for memory optimization."""
    
    @staticmethod
    def estimate_object_size(obj: Any) -> int:
        """Estimate the memory size of an object."""
        import sys
        
        size = sys.getsizeof(obj)
        
        # Add size of attributes for dataclass objects
        if hasattr(obj, '__dict__'):
            for value in obj.__dict__.values():
                size += sys.getsizeof(value)
        
        # Add size of slots if present
        if hasattr(obj, '__slots__'):
            for slot in obj.__slots__:
                if hasattr(obj, slot):
                    size += sys.getsizeof(getattr(obj, slot))
        
        return size
    
    @staticmethod
    def get_cache_memory_usage(season: 'Season') -> Dict[str, int]:
        """Get memory usage breakdown for a season."""
        from .season import Season
        
        total_size = MemoryOptimizer.estimate_object_size(season)
        
        # Estimate sizes of collections
        events_size = sum(
            MemoryOptimizer.estimate_object_size(event) 
            for event in season.events.values()
        )
        
        teams_size = sum(
            MemoryOptimizer.estimate_object_size(team) 
            for team in season.teams.values()
        )
        
        return {
            "total_size": total_size,
            "events_size": events_size,
            "teams_size": teams_size,
            "overhead_size": total_size - events_size - teams_size
        }


# Global performance monitor instance
performance_monitor = PerformanceMonitor()


def optimized_search(items: List[T], key_func: Callable[[T], Any], 
                    search_value: Any) -> Optional[T]:
    """
    Optimized search function using binary search when possible.
    """
    if not items:
        return None
    
    # For small collections, linear search is faster
    if len(items) < 20:
        for item in items:
            if key_func(item) == search_value:
                return item
        return None
    
    # For larger collections, try sorted search if keys are comparable
    try:
        sorted_items = sorted(items, key=key_func)
        # Binary search implementation
        left, right = 0, len(sorted_items) - 1
        
        while left <= right:
            mid = (left + right) // 2
            mid_value = key_func(sorted_items[mid])
            
            if mid_value == search_value:
                return sorted_items[mid]
            elif mid_value < search_value:
                left = mid + 1
            else:
                right = mid - 1
        
        return None
    except (TypeError, AttributeError):
        # Fall back to linear search if keys aren't comparable
        for item in items:
            if key_func(item) == search_value:
                return item
        return None


def memoize_method(func: Callable[..., T]) -> Callable[..., T]:
    """
    Decorator to memoize instance methods with proper cache invalidation.
    """
    cache_attr_name = f"_memoize_cache_{func.__name__}"
    
    @wraps(func)
    def wrapper(self, *args, **kwargs):
        # Get or create cache for this instance
        if not hasattr(self, cache_attr_name):
            setattr(self, cache_attr_name, {})
        
        cache = getattr(self, cache_attr_name)
        
        # Create cache key from args and kwargs
        key = (args, tuple(sorted(kwargs.items())))
        
        if key not in cache:
            cache[key] = func(self, *args, **kwargs)
        
        return cache[key]
    
    return wrapper
